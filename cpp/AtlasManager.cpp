#include <iostream>

#include <bsoncxx/builder/stream/document.hpp>
#include <bsoncxx/json.hpp>
#include <bsoncxx/builder/basic/document.hpp>
#include <bsoncxx/builder/basic/kvp.hpp>

#include "AtlasManager.h"

using bsoncxx::builder::basic::kvp;
using bsoncxx::builder::basic::make_document;

AtlasManager* AtlasManager::sInstance = NULL;

AtlasManager* AtlasManager::Instance() {

	if(sInstance == NULL) {
    // mongocxx::instance inst{};
    sInstance = new AtlasManager();
  }

	return sInstance;
}

mongocxx::collection AtlasManager::getCollection(std::string dbname, std::string collname) {
  auto client = _connectionPool.acquire();
  mongocxx::database db = (*client)[dbname];
  mongocxx::collection collection = db[collname];

  return collection;
}

void AtlasManager::WritePlayerMove(Vec2 position, Vec2 velocity) {
  auto client = _connectionPool.acquire();
  mongocxx::database db = (*client)["test"];
  mongocxx::collection collection = db["moves"];

  collection.insert_one(
        make_document(
            kvp("gameId", getEnvVar("PONG_GAME_ID")),
            kvp("playerId", getEnvVar("PONG_PLAYER_ID")),
            kvp("position", make_document(kvp("x", position.x), kvp("y", position.y))),
            kvp("velocity", make_document(kvp("x", velocity.x), kvp("y", velocity.y)))
        )
    );
}


AtlasManager::AtlasManager() {
  bsoncxx::builder::stream::document document{};

  auto client = _connectionPool.acquire();
  auto collection = (*client)["test"]["foo"];
  document << "hello" << "world";

  collection.insert_one(document.view());
}

void AtlasManager::TestConnection() {
  auto client = _connectionPool.acquire();
  auto collection = (*client)["test"]["foo"];
  auto cursor = collection.find({});

  for (auto&& doc : cursor) {
      std::cout << bsoncxx::to_json(doc) << std::endl;
  }
}

AtlasManager::~AtlasManager() {
	delete sInstance;
	sInstance = NULL;
}