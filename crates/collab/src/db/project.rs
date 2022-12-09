use super::{ProjectId, RoomId, UserId};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ProjectId,
    pub room_id: RoomId,
    pub host_user_id: UserId,
    pub host_connection_id: i32,
    pub host_connection_epoch: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::HostUserId",
        to = "super::user::Column::Id"
    )]
    HostUser,
    #[sea_orm(
        belongs_to = "super::room::Entity",
        from = "Column::RoomId",
        to = "super::room::Column::Id"
    )]
    Room,
    #[sea_orm(has_many = "super::worktree::Entity")]
    Worktrees,
    #[sea_orm(has_many = "super::project_collaborator::Entity")]
    Collaborators,
    #[sea_orm(has_many = "super::language_server::Entity")]
    LanguageServers,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HostUser.def()
    }
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl Related<super::worktree::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Worktrees.def()
    }
}

impl Related<super::project_collaborator::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collaborators.def()
    }
}

impl Related<super::language_server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LanguageServers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}