#include <iostream>

#include <bsoncxx/builder/stream/document.hpp>
#include <bsoncxx/json.hpp>

#include "AtlasManager.h"

AtlasManager* AtlasManager::sInstance = NULL;

AtlasManager* AtlasManager::Instance() {

	if(sInstance == NULL)
		sInstance = new AtlasManager();

	return sInstance;

}

AtlasManager::AtlasManager() {
  bsoncxx::builder::stream::document document{};

  auto collection = _mongoClient["test"]["foo"];
  document << "hello" << "world";

  collection.insert_one(document.view());
}

void AtlasManager::TestConnection() {
  auto collection = _mongoClient["test"]["foo"];
  auto cursor = collection.find({});

  for (auto&& doc : cursor) {
      std::cout << bsoncxx::to_json(doc) << std::endl;
  }
}

AtlasManager::~AtlasManager() {

	delete sInstance;
	sInstance = NULL;
}