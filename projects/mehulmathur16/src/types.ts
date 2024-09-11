import DataLoader from "dataloader";

export interface IContext {
    userDataLoader: DataLoader<unknown, unknown, unknown>,
    postDataLoader: DataLoader<unknown, unknown, unknown>,
    cache: {
        usersList: any[],
        postsList: any[],
        postMap: Record<any, any>,
        userMap: Record<any, any>
    }
}