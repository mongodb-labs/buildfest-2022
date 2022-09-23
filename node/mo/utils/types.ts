import { NextApiHandler } from "next"
import { EJSON } from "bson";

/// A set of HTTP-method-handling functions
export interface MethodResponder {
    get?: NextApiHandler;
    post?: NextApiHandler;
    put?: NextApiHandler;
    delete?: NextApiHandler;
}

/**
 * The type of document that contains aliased links
 */
export interface MoLink {
    /// The spelling of the alias of the link
    alias: string;
    /// The target of the link
    link: string;
    /// The number of times the link has been used
    n: number;
    /// The time at which this object was created
    createdAt: Date;
    // is a Regex redirect
    isRegex: boolean;
}

/// Type of the data object sent to insert/update a link
export type MoLinkPutJSON = Partial<Omit<MoLink, 'createdAt' | 'n'>>;

export interface ExtendedJSONEncoded<T> {
    __ejson__: T & { __: never };
}

export function encodeExtendedJSON<T extends EJSON.SerializableTypes>(val: T): ExtendedJSONEncoded<T> {
    const v = EJSON.serialize(val);
    return v as unknown as ExtendedJSONEncoded<T>;
}

export function decodedExtendedJSON<T>(encoded: ExtendedJSONEncoded<T>): T {
    const v = EJSON.deserialize(encoded);
    return v as unknown as T;
}
