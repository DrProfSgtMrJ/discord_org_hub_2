use entity::discord_user::{self};
use entity::member::{
    self, ActiveModel as MemberActiveModel, Entity as MemberEntity, Model as MemberModel,
};
use entity::season_member::{ActiveModel as SeasonMemberActiveModel, Model as SeasonMemberModel};
use entity::{org, season_member};
use sea_orm::sea_query::NullOrdering;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
    RelationTrait,
};
use uuid::Uuid;

use crate::{DbService, OrderBy};

#[derive(Debug, Clone)]
pub struct MemberWithName {
    pub member: MemberModel,
    pub display_name: Option<String>,
    pub discord_user_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SeasonMemberWithName {
    pub season_member: SeasonMemberModel,
    pub member: Option<MemberModel>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[async_trait::async_trait]
pub trait MemberService {
    async fn create_member(&self, user_id: Uuid, org_id: Uuid) -> Result<MemberModel, DbErr>;
    async fn get_members_in_org(&self, org_id: Uuid) -> Result<Vec<MemberModel>, DbErr>;
    async fn get_members_with_names_in_org(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<MemberWithName>, DbErr>;
    async fn get_member_by_ids(
        &self,
        discord_user_id: &str,
        discord_guild_id: &str,
    ) -> Result<Option<MemberModel>, DbErr>;
    async fn add_member_to_season(
        &self,
        member_id: Uuid,
        season_id: Uuid,
        placement: Option<i32>,
    ) -> Result<SeasonMemberModel, DbErr>;

    async fn get_members_in_season(
        &self,
        season_id: Uuid,
        order_by: Option<OrderBy<season_member::Column>>,
    ) -> Result<Vec<SeasonMemberWithName>, DbErr>;
}

#[async_trait::async_trait]
impl MemberService for DbService {
    async fn create_member(&self, user_id: Uuid, org_id: Uuid) -> Result<MemberModel, DbErr> {
        let member = MemberActiveModel::new(user_id, org_id);

        match self.get_connection() {
            Ok(conn) => member.insert(conn).await,
            Err(err) => Err(err),
        }
    }

    async fn get_members_in_org(&self, org_id: Uuid) -> Result<Vec<MemberModel>, DbErr> {
        match self.get_connection() {
            Ok(conn) => {
                MemberEntity::find()
                    .filter(member::Column::OrgId.eq(org_id))
                    .all(conn)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    async fn get_member_by_ids(
        &self,
        discord_user_id: &str,
        discord_guild_id: &str,
    ) -> Result<Option<MemberModel>, DbErr> {
        match self.get_connection() {
            Ok(conn) => {
                member::Entity::find()
                    .join(
                        sea_orm::JoinType::InnerJoin,
                        member::Relation::DiscordUser.def(),
                    )
                    .join(sea_orm::JoinType::InnerJoin, member::Relation::Org.def())
                    .filter(discord_user::Column::DiscordId.eq(discord_user_id))
                    .filter(org::Column::DiscordId.eq(discord_guild_id))
                    .one(conn)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    async fn add_member_to_season(
        &self,
        member_id: Uuid,
        season_id: Uuid,
        placement: Option<i32>,
    ) -> Result<SeasonMemberModel, DbErr> {
        let season_member = SeasonMemberActiveModel::new(season_id, member_id, placement);
        match self.get_connection() {
            Ok(conn) => season_member.insert(conn).await,
            Err(err) => Err(err),
        }
    }

    async fn get_members_in_season(
        &self,
        season_id: Uuid,
        order_by: Option<OrderBy<season_member::Column>>,
    ) -> Result<Vec<SeasonMemberWithName>, DbErr> {
        match self.get_connection() {
            Ok(conn) => {
                let results = match order_by {
                    Some(OrderBy::Asc { column }) => {
                        season_member::Entity::find()
                            .filter(season_member::Column::SeasonId.eq(season_id))
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                season_member::Relation::Member.def(),
                            )
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                member::Relation::DiscordUser.def(),
                            )
                            .select_also(member::Entity)
                            .select_also(discord_user::Entity)
                            .order_by_asc(column)
                            .all(conn)
                            .await
                    }
                    Some(OrderBy::Desc { column }) => {
                        season_member::Entity::find()
                            .filter(season_member::Column::SeasonId.eq(season_id))
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                season_member::Relation::Member.def(),
                            )
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                member::Relation::DiscordUser.def(),
                            )
                            .select_also(member::Entity)
                            .select_also(discord_user::Entity)
                            .order_by_desc(column)
                            .all(conn)
                            .await
                    }
                    Some(OrderBy::AscNullsFirst { column }) => {
                        season_member::Entity::find()
                            .filter(season_member::Column::SeasonId.eq(season_id))
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                season_member::Relation::Member.def(),
                            )
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                member::Relation::DiscordUser.def(),
                            )
                            .select_also(member::Entity)
                            .select_also(discord_user::Entity)
                            .order_by_with_nulls(column, sea_orm::Order::Asc, NullOrdering::First)
                            .all(conn)
                            .await
                    }
                    Some(OrderBy::DescNullsFirst { column }) => {
                        season_member::Entity::find()
                            .filter(season_member::Column::SeasonId.eq(season_id))
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                season_member::Relation::Member.def(),
                            )
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                member::Relation::DiscordUser.def(),
                            )
                            .select_also(member::Entity)
                            .select_also(discord_user::Entity)
                            .order_by_with_nulls(column, sea_orm::Order::Desc, NullOrdering::First)
                            .all(conn)
                            .await
                    }
                    Some(OrderBy::DescNullsLast { column }) => {
                        season_member::Entity::find()
                            .filter(season_member::Column::SeasonId.eq(season_id))
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                season_member::Relation::Member.def(),
                            )
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                member::Relation::DiscordUser.def(),
                            )
                            .select_also(member::Entity)
                            .select_also(discord_user::Entity)
                            .order_by_with_nulls(column, sea_orm::Order::Desc, NullOrdering::Last)
                            .all(conn)
                            .await
                    }
                    Some(OrderBy::AscNullsLast { column }) => {
                        season_member::Entity::find()
                            .filter(season_member::Column::SeasonId.eq(season_id))
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                season_member::Relation::Member.def(),
                            )
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                member::Relation::DiscordUser.def(),
                            )
                            .select_also(member::Entity)
                            .select_also(discord_user::Entity)
                            .order_by_with_nulls(column, sea_orm::Order::Asc, NullOrdering::Last)
                            .all(conn)
                            .await
                    }
                    None => {
                        season_member::Entity::find()
                            .filter(season_member::Column::SeasonId.eq(season_id))
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                season_member::Relation::Member.def(),
                            )
                            .join(
                                sea_orm::JoinType::InnerJoin,
                                member::Relation::DiscordUser.def(),
                            )
                            .select_also(member::Entity)
                            .select_also(discord_user::Entity)
                            .all(conn)
                            .await
                    }
                };
                if let Ok(res) = results {
                    let members: Vec<SeasonMemberWithName> = res
                        .into_iter()
                        .map(
                            |(season_member, member, discord_user)| SeasonMemberWithName {
                                season_member,
                                member,
                                display_name: discord_user.clone().map(|user| user.display_name),
                                avatar_url: discord_user.clone().map(|user| user.avatar_url),
                            },
                        )
                        .collect();
                    return Ok(members);
                }
                Ok(vec![])
            }
            Err(err) => Err(err),
        }
    }

    async fn get_members_with_names_in_org(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<MemberWithName>, DbErr> {
        match self.get_connection() {
            Ok(conn) => {
                let results = MemberEntity::find()
                    .filter(member::Column::OrgId.eq(org_id))
                    .join(
                        sea_orm::JoinType::InnerJoin,
                        member::Relation::DiscordUser.def(),
                    )
                    .select_also(discord_user::Entity)
                    .all(conn)
                    .await;

                if let Ok(res) = results {
                    let members: Vec<MemberWithName> = res
                        .into_iter()
                        .map(|(member, discord_user)| MemberWithName {
                            member,
                            display_name: discord_user.as_ref().map(|u| u.display_name.clone()),
                            discord_user_id: discord_user.as_ref().map(|u| u.discord_id.clone()),
                        })
                        .collect();
                    return Ok(members);
                }
                Ok(vec![])
            }
            Err(err) => Err(err),
        }
    }
}
