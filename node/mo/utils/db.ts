import { MongoClient, Collection, Db, Document, AbstractCursor } from "mongodb";
import { MOLINKS_CONFIG } from "./config";
import { MoLink } from "./types";

/**
 * @returns A new MongoClient connected with the default database URL.
 *
 * @note This will need to be closed!
 */
export async function newMongoClient(): Promise<MongoClient> {
    return MongoClient.connect(MOLINKS_CONFIG.DATABASE_URL, {
        appName: 'molinks',
        connectTimeoutMS: 1_000,
    });
}

let _DEFAULT_CLIENT: Promise<MongoClient> | undefined;
/**
 * @returns The application-wide default MongoDB client
 */
export async function getDefaultClient(): Promise<MongoClient> {
    if (_DEFAULT_CLIENT === undefined) {
        _DEFAULT_CLIENT = newMongoClient();
    }
    return await _DEFAULT_CLIENT;
}

/**
 * @param client An optional client to use
 * @returns {Db} The default database for the client
 */
export async function openDefaultDB(client?: MongoClient): Promise<Db> {
    client ??= await getDefaultClient();
    return client.db('molinks');
}

/**
 * @param name The name of the collection to open
 * @returns A new handle to the collection
 */
export async function openCollection<T extends Document>(name: string, db: Db): Promise<Collection<T>>;
export async function openCollection<T extends Document>(name: string, client: MongoClient): Promise<Collection<T>>;
export async function openCollection<T extends Document>(name: string, dbOrClient: Db | MongoClient): Promise<Collection<T>> {
    if (dbOrClient instanceof MongoClient) {
        dbOrClient = await openDefaultDB(dbOrClient);
    }
    return dbOrClient.collection<T>(name);
}

/**
 * Opne the MoLink collection in the database
 *
 * @param db Override the database to connect to
 * @returns A handle to the collection of MoLink objects
 */
export async function openLinksCollection(db?: Db): Promise<Collection<MoLink>> {
    db ??= await openDefaultDB();
    return openCollection<MoLink>('links', db);
}
