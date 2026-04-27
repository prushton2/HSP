import type { JSX } from "react";
import { toast, ToastContainer as Tc, Slide} from "react-toastify";
import type { Result } from "./Result";

export function ToastContainer(): JSX.Element {
    return <Tc 
        position="bottom-right"
        autoClose={5000}
        hideProgressBar={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        theme="dark"
        transition={Slide}
    />
}

export namespace Toast {
    export function success(message: string) {
        toast.success(message);
    }

    export function error(message: string) {
        toast.error(message);
    }

    export function info(message: string) {
        toast.info(message);
    }

    export async function WrapFunction<T, E>(fn: () => Promise<Result<T, E>>, message: string | null = null): Promise<Result<T, E>> {
        let result = await fn();

        if(result.is_ok()) {
            success(message != null ? message : "Success");
        } else {
            error(result.into_err() as string);
        }

        return result;
    }

    export async function WrapErr<T, E>(fn: () => Promise<Result<T, E>>): Promise<Result<T, E>> {
        let result = await fn();

        if(result.is_err()) {
            error(result.into_err() as string);
        }

        return result;
    }
}
