use std::sync::Mutex;
use async_trait::async_trait;
use crate::internal::domain::entities::user_entity::UserEntity;
use crate::internal::domain::repository_interface::user_repository::UserRepository;

// make inmemory list
pub struct UserRepositoryImpl {
    users: std::sync::Mutex<Vec<UserEntity>>,
}

impl UserRepositoryImpl {
    // create user repository
    pub fn new() -> Self {
        Self {
            users: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    fn new() -> Self {
        UserRepositoryImpl::new()
    }

    async fn create_name(&self, user: UserEntity) -> UserEntity {
        let mut users = self.users.lock().unwrap();
        users.push(user.clone());
        user
    }

    async fn get_names(&self) -> Vec<UserEntity> {
        let names = self.users.lock().unwrap();
        names.clone()
    }

    // async fn get_names(&self) -> Vec<String> {
    //     let names = self.users.lock().unwrap();
    //     names.iter().map(|user| user.name.clone()).collect()
    // }
    async fn delete_index(&self, index: u32) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        let index = index as usize;

        if index >= users.len() {
            return Err(format!("Index out of bounds: {}", index));
        }

        users.remove(index);
        Ok(())
    }

    async fn delete_name(&self, name: String) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();

        // search index
        let position = users.iter().position(|user| user.name == name);

        // if not found return err or delete
        match position {
            Some(index) => {
                users.remove(index);
                Ok(())
            },
            None => Err(format!("User not found: {}", name))
        }
    }
}
