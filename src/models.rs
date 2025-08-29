use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User{
   pub id:i32,
   pub name:String,
   pub email:String
}

#[derive(Serialize, Deserialize, Debug )]
pub struct NewUser{
  pub  name:String, 
   pub email:String

}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task{
   pub id:i32,
  pub  user_id:i32,
   pub title:String,
   pub completed:bool
}


#[derive(Serialize, Deserialize,Debug)]
pub struct NewTask{
    pub title:String,
    pub user_id:i32,
    pub completed:bool
}