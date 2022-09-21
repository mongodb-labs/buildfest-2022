// Interface to defining our object of response functions
export interface ResponseFuncs {
    GET?: Function
    POST?: Function
    PUT?: Function
    DELETE?: Function
}

// Interface to define our MoLink model on the frontend
export interface MoLink {
    _id?: number
    alias: string
    link: string
    n: number
    createdAt: Date
}
