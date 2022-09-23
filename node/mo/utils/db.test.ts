import { MongoClient, Db } from "mongodb";

import { suite, test, Context, suiteSetup, setup, suiteTeardown, teardown } from 'mocha';
import { expect } from 'chai';

import * as dbmod from "./db";
import { MoLink } from "./types";

function _requireDatabaseURL(ctx: Context) {
  // The DATABASE_URL is used to open a client in newMongoClient()
  if (!('DATABASE_URL' in process.env)) {
    ctx.skip();
  }
}

/**
 * These tests don't do much interesting, just exercising the driver and the simple wrapping APIs
 */
suite('With a client', function () {
  // Check that we have a DATABASE_URL set, or skip the tests
  this.beforeAll('Require databases', function () { _requireDatabaseURL(this); });

  let cl: MongoClient;
  suiteSetup('Open a client', async () => {
    cl = await dbmod.newMongoClient();
  });
  suiteTeardown('Close a client', async () => {
    if (cl) await cl.close();
  });

  suite('With a database', () => {
    let db: Db;
    setup('Create database', () => {
      db = cl.db('test-molinks');
    });
    teardown('Cleanup the database', async () => {
      if (db) await db.dropDatabase();
    });

    test('Create the default collection', async () => {
      const coll = await dbmod.openCollection<MoLink>('default', db);
      // Default is empty
      const cursor = coll.find();
      const doc = await cursor.next();
      expect(doc, 'expect empty collection').to.be.null;
    });

    test('Insert a link', async () => {
      const coll = await dbmod.openCollection<MoLink>('links', db);
      const result = await coll.insertOne({
        alias: 'widgets',
        link: 'gadgets',
        n: 1,
        createdAt: new Date(),
      });
      expect(result.acknowledged).to.be.true;
      const recover = await coll.findOne({ _id: result.insertedId });
      expect(recover).to.not.be.null;
      expect(recover?.alias).to.eql('widgets');
      expect(recover?._id.id).to.eql(result.insertedId.id);
      expect(recover?.n).to.eq(1);
    });
  });
});
