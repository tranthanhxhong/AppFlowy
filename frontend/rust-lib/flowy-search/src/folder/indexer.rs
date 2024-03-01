use std::{any::Any, collections::HashMap, fs, path::Path, sync::Weak};

use crate::{
  entities::ResultIconTypePB,
  folder::schema::{FolderSchema, FOLDER_ICON_FIELD_NAME, FOLDER_TITLE_FIELD_NAME},
};
use collab::core::collab::{IndexContent, IndexContentReceiver};
use collab_folder::{View, ViewIcon, ViewIndexContent, ViewLayout};
use flowy_error::{FlowyError, FlowyResult};
use flowy_user::services::authenticate_user::AuthenticateUser;
use lib_dispatch::prelude::af_spawn;
use strsim::levenshtein;
use tantivy::{
  collector::TopDocs, directory::MmapDirectory, doc, query::QueryParser, Index, IndexReader,
  IndexWriter, Term,
};

use crate::{
  entities::SearchResultPB,
  services::indexer::{IndexManager, IndexableData},
};

use super::{
  entities::FolderIndexData,
  schema::{FOLDER_ICON_TY_FIELD_NAME, FOLDER_ID_FIELD_NAME},
};

#[derive(Clone)]
pub struct FolderIndexManager {
  folder_schema: Option<FolderSchema>,
  index: Option<Index>,
  index_reader: Option<IndexReader>,
}

const FOLDER_INDEX_DIR: &str = "folder_index";

impl FolderIndexManager {
  pub fn new(auth_user: Weak<AuthenticateUser>) -> Self {
    let authenticate_user = auth_user.upgrade();
    let storage_path = match authenticate_user {
      Some(auth_user) => auth_user.get_index_path(),
      None => {
        tracing::error!("FolderIndexManager: AuthenticateUser is not available");
        return FolderIndexManager::empty();
      },
    };

    let index_path = storage_path.join(Path::new(FOLDER_INDEX_DIR));
    if !index_path.exists() {
      let res = fs::create_dir_all(&index_path);
      if let Err(e) = res {
        tracing::error!(
          "FolderIndexManager failed to create index directory: {:?}",
          e
        );
        return FolderIndexManager::empty();
      }
    }

    let dir = MmapDirectory::open(index_path);
    if let Err(e) = dir {
      tracing::error!("FolderIndexManager failed to open index directory: {:?}", e);
      return FolderIndexManager::empty();
    }

    let folder_schema = FolderSchema::new();
    let index_res = Index::open_or_create(dir.unwrap(), folder_schema.clone().schema);
    if let Err(e) = index_res {
      tracing::error!("FolderIndexManager failed to open index: {:?}", e);
      return FolderIndexManager::empty();
    }

    let index = index_res.unwrap();
    let index_reader = index.reader();
    if let Err(e) = index_reader {
      tracing::error!(
        "FolderIndexManager failed to instantiate index reader: {:?}",
        e
      );
      return FolderIndexManager::empty();
    }

    Self {
      folder_schema: Some(folder_schema),
      index: Some(index),
      index_reader: Some(index_reader.unwrap()),
    }
  }

  pub fn index_all_views(&self, views: Vec<View>) {
    let indexable_data = views
      .iter()
      .map(|view| IndexableData {
        id: view.id.clone(),
        data: view.name.clone(),
        icon: view.icon.clone(),
        layout: view.layout.clone(),
      })
      .collect();

    let _ = self.index_all(indexable_data);
  }

  fn index_all(&self, indexes: Vec<IndexableData>) -> Result<(), FlowyError> {
    if self.is_indexed() || indexes.is_empty() {
      return Ok(());
    }

    let mut index_writer = self.get_index_writer()?;
    let folder_schema = self.get_folder_schema()?;

    let id_field = folder_schema.schema.get_field(FOLDER_ID_FIELD_NAME)?;
    let title_field = folder_schema.schema.get_field(FOLDER_TITLE_FIELD_NAME)?;
    let icon_field = folder_schema.schema.get_field(FOLDER_ICON_FIELD_NAME)?;
    let icon_ty_field = folder_schema.schema.get_field(FOLDER_ICON_TY_FIELD_NAME)?;

    for data in indexes {
      let (icon, icon_ty) = self.extract_icon(data.icon.clone(), data.layout.clone());

      let _ = index_writer.add_document(doc![
      id_field => data.id.clone(),
      title_field => data.data.clone(),
      icon_field => icon.unwrap_or_default(),
      icon_ty_field => icon_ty,
      ]);
    }

    index_writer.commit()?;

    Ok(())
  }

  pub fn is_indexed(&self) -> bool {
    if let Some(index) = &self.index {
      let index_reader = index.reader();
      if let Ok(index_reader) = index_reader {
        let searcher = index_reader.searcher();
        let num_docs = searcher.num_docs();
        if num_docs > 0 {
          return true;
        }
      }
    }

    false
  }

  fn empty() -> Self {
    Self {
      folder_schema: None,
      index: None,
      index_reader: None,
    }
  }

  fn get_index_writer(&self) -> FlowyResult<IndexWriter> {
    match &self.index {
      // Creates an IndexWriter with a heap size of 50 MB (50.000.000 bytes)
      Some(index) => Ok(index.writer(50_000_000)?),
      None => Err(FlowyError::folder_index_manager_unavailable()),
    }
  }

  fn get_folder_schema(&self) -> FlowyResult<FolderSchema> {
    match &self.folder_schema {
      Some(folder_schema) => Ok(folder_schema.clone()),
      None => Err(FlowyError::folder_index_manager_unavailable()),
    }
  }

  fn extract_icon(
    &self,
    view_icon: Option<ViewIcon>,
    view_layout: ViewLayout,
  ) -> (Option<String>, i64) {
    let icon_ty: i64;
    let icon: Option<String>;

    if let Some(view_icon) = view_icon {
      let result_icon_ty: ResultIconTypePB = view_icon.ty.into();
      icon_ty = result_icon_ty.into();
      icon = Some(view_icon.value);
    } else {
      icon_ty = ResultIconTypePB::Icon.into();
      let layout_ty: i64 = view_layout.into();
      icon = Some(layout_ty.to_string());
    }

    (icon, icon_ty)
  }

  pub fn search(&self, query: String) -> Result<Vec<SearchResultPB>, FlowyError> {
    let folder_schema = self.get_folder_schema()?;

    let index = match &self.index {
      Some(index) => index,
      None => return Err(FlowyError::folder_index_manager_unavailable()),
    };

    let index_reader = match &self.index_reader {
      Some(index_reader) => index_reader,
      None => return Err(FlowyError::folder_index_manager_unavailable()),
    };

    let title_field = folder_schema.schema.get_field(FOLDER_TITLE_FIELD_NAME)?;

    let length = query.len();
    let distance: u8 = match length {
      _ if length > 4 => 2,
      _ if length > 2 => 1,
      _ => 0,
    };

    let mut query_parser = QueryParser::for_index(&index.clone(), vec![title_field]);
    query_parser.set_field_fuzzy(title_field, true, distance, true);
    let built_query = query_parser.parse_query(&query.clone())?;

    let searcher = index_reader.searcher();
    let mut search_results: Vec<SearchResultPB> = vec![];
    let top_docs = searcher.search(&built_query, &TopDocs::with_limit(10))?;
    for (_score, doc_address) in top_docs {
      let retrieved_doc = searcher.doc(doc_address)?;

      let mut content = HashMap::new();
      let named_doc = folder_schema.schema.to_named_doc(&retrieved_doc);
      for (k, v) in named_doc.0 {
        content.insert(k, v[0].clone());
      }

      if content.is_empty() {
        continue;
      }

      let s = serde_json::to_string(&content)?;
      let result: SearchResultPB = serde_json::from_str::<FolderIndexData>(&s)?.into();
      let score = self.score_result(&query, &result.data);
      search_results.push(result.with_score(score));
    }

    Ok(search_results)
  }

  // Score result by distance
  fn score_result(&self, query: &str, term: &str) -> f64 {
    let distance = levenshtein(query, term) as f64;
    1.0 / (distance + 1.0)
  }
}

impl IndexManager for FolderIndexManager {
  fn set_index_content_receiver(&self, mut rx: IndexContentReceiver) {
    let indexer = self.clone();
    af_spawn(async move {
      while let Ok(msg) = rx.recv().await {
        match msg {
          IndexContent::Create(value) => match serde_json::from_value::<ViewIndexContent>(value) {
            Ok(view) => {
              let _ = indexer.add_index(IndexableData {
                id: view.id,
                data: view.name,
                icon: view.icon,
                layout: view.layout,
              });
            },
            Err(err) => tracing::error!("FolderIndexManager error deserialize: {:?}", err),
          },
          IndexContent::Update(value) => match serde_json::from_value::<ViewIndexContent>(value) {
            Ok(view) => {
              let _ = indexer.update_index(IndexableData {
                id: view.id,
                data: view.name,
                icon: view.icon,
                layout: view.layout,
              });
            },
            Err(err) => tracing::error!("FolderIndexManager error deserialize: {:?}", err),
          },
          IndexContent::Delete(ids) => {
            if let Err(e) = indexer.remove_indices(ids) {
              tracing::error!("FolderIndexManager error deserialize: {:?}", e);
            }
          },
        }
      }
    });
  }

  fn update_index(&self, data: IndexableData) -> Result<(), FlowyError> {
    let mut index_writer = self.get_index_writer()?;

    let folder_schema = self.get_folder_schema()?;
    let id_field = folder_schema.schema.get_field(FOLDER_ID_FIELD_NAME)?;
    let title_field = folder_schema.schema.get_field(FOLDER_TITLE_FIELD_NAME)?;
    let icon_field = folder_schema.schema.get_field(FOLDER_ICON_FIELD_NAME)?;
    let icon_ty_field = folder_schema.schema.get_field(FOLDER_ICON_TY_FIELD_NAME)?;

    let delete_term = Term::from_field_text(id_field, &data.id.clone());

    // Remove old index
    index_writer.delete_term(delete_term);

    let (icon, icon_ty) = self.extract_icon(data.icon, data.layout);

    // Add new index
    let _ = index_writer.add_document(doc![
      id_field => data.id.clone(),
      title_field => data.data,
      icon_field => icon.unwrap_or_default(),
      icon_ty_field => icon_ty,
    ]);

    index_writer.commit()?;

    Ok(())
  }

  fn remove_indices(&self, ids: Vec<String>) -> Result<(), FlowyError> {
    let mut index_writer = self.get_index_writer()?;
    let folder_schema = self.get_folder_schema()?;

    let id_field = folder_schema.schema.get_field(FOLDER_ID_FIELD_NAME)?;
    for id in ids {
      let delete_term = Term::from_field_text(id_field, &id);
      index_writer.delete_term(delete_term);
    }

    index_writer.commit()?;

    Ok(())
  }

  fn add_index(&self, data: IndexableData) -> Result<(), FlowyError> {
    let mut index_writer = self.get_index_writer()?;

    let folder_schema = self.get_folder_schema()?;

    let id_field = folder_schema.schema.get_field(FOLDER_ID_FIELD_NAME)?;
    let title_field = folder_schema.schema.get_field(FOLDER_TITLE_FIELD_NAME)?;
    let icon_field = folder_schema.schema.get_field(FOLDER_ICON_FIELD_NAME)?;
    let icon_ty_field = folder_schema.schema.get_field(FOLDER_ICON_TY_FIELD_NAME)?;

    let (icon, icon_ty) = self.extract_icon(data.icon, data.layout);

    // Add new index
    let _ = index_writer.add_document(doc![
      id_field => data.id,
      title_field => data.data,
      icon_field => icon.unwrap_or_default(),
      icon_ty_field => icon_ty,
    ]);

    index_writer.commit()?;

    Ok(())
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}
