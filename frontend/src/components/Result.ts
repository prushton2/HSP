export class Result<T, E> {
    data: T | E | null = null;
    tag: ResultTag = ResultTag.Err;

    is_ok (): boolean {
        return this.tag == ResultTag.Ok
    }

    is_err (): boolean {
        return this.tag == ResultTag.Err
    }

    into_ok (): T {
        return this.data as T
    }

    into_err (): E {
        return this.data as E
    }
}

export namespace NewResult {
    export function from_ok<T, E> (ok: T): Result<T, E> {
        let i = new Result<T, E>();
        i.data = ok;
        i.tag = ResultTag.Ok;
        return i;
    }

    export function from_err<T, E> (err: E): Result<T, E> {
        let i = new Result<T, E>();
        i.data = err;
        i.tag = ResultTag.Err;
        return i;
    }
}

enum ResultTag {
    Ok,
    Err
}