use super::entities::{InsertItemRequest, Item, ItemBson}; // ใช้ super เพื่อบอก rust ว่าให้มองหา file นี้ใน parent directory
use crate::config::database::dbconnect;
// ใช้ crate เพื่อบอก rust ว่าให้มองหา file นี้ใน root directory
use bson::{doc, from_document, oid::ObjectId, Document};
use mongodb::{error::Error, results::InsertOneResult};
use tracing::log::info;

pub async fn insert_one_item(req: InsertItemRequest) -> Result<ObjectId, String> {
    // Connected to database
    let db = dbconnect().await.expect("error connection to database");
    let col = db.collection::<Document>("items");

    let result: Result<InsertOneResult, Error> = col
        .insert_one(doc! {
            "name": req.name,
            "description": req.description,
            "damage": req.damage,
            "level_required": req.level_required,
            "price": req.price,
        })
        .await;

    let inserted_id_bson = match result {
        Ok(inserted_id) => inserted_id.inserted_id,
        Err(e) => {
            info!("Error: {}", e);
            return Err(format!("Error: Insert one item failed"));
        }
    };

    // จัดการ Result ได้ทั้ง 2 แบบ
    // แบบที่ 1 ใช้ let if
    if let bson::Bson::ObjectId(id) = inserted_id_bson {
        info!("Inserted id: {}", id);
        return Ok(id);
    } else {
        return Err(format!("Error: Inserted id is not ObjectId"));
    }

    // แบบที่ 2 ใช้ match
    // match inserted_id_bson {
    //     bson::Bson::ObjectId(id) => {
    //         info!("Inserted id: {}", id);
    //         return Ok(id);
    //     }
    //     _ => {
    //         return Err(format!("Error: Inserted id is not ObjectId"));
    //     }
    // }
}

pub async fn find_one_item(item_id: ObjectId) -> Result<Item, String> {
    // Connected to database
    let db = dbconnect().await.expect("error connection to database");
    let col = db.collection::<Document>("items");

    let cursor = col.find_one(doc! {"_id": item_id}).await;

    // ถอนข้อมุล doc ออกจาก Result
    let document = match cursor {
        Ok(r) => r,
        Err(err) => {
            info!("Error: {}", err);
            return Err(format!("Error: Find one item failed"));
        }
    };

    // ถอนข้อมูลจาก doc ออกมา
    let item: ItemBson = match document {
        Some(r) => match from_document::<ItemBson>(r) {
            Ok(i) => i, // ถ้าสำเร็จ คืนค่า i
            Err(e) => {
                info!("Error: {}", e);
                return Err(format!("Error: Find one item failed"));
            }
        },
        None => {
            return Err(format!("Error: Find one item failed"));
        }
    };

    Ok(Item {
        _id: item._id.to_hex(),
        name: item.name,
        description: item.description,
        damage: item.damage,
        level_required: item.level_required,
        price: item.price,
    })
}
