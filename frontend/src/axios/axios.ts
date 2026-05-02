import axios from "axios";
import type { ApiRequestObjects, ApiResponseObjects, Tables } from "./structs";
import { NewResult, Result } from "../components/Result";

export namespace Http {
    export namespace Admin {
        export async function GetAllTables(): Promise<ApiResponseObjects.AllTables> {
            const response = await axios.get('api/admin/all');
            return response.data as ApiResponseObjects.AllTables
        }
    }

    export async function Self(): Promise<Result<Tables.Users, string>> {
            try {
                let response = await axios.get("/api/auth/self");
                return NewResult.from_ok(response.data);
            } catch (err) {
                if(!axios.isAxiosError(err)) { return NewResult.from_err("Non Axios Error"); }
                return NewResult.from_err(err.response?.data);
            }
        }
    

    export namespace Student {
        export async function Create(student: ApiRequestObjects.CreateStudent): Promise<Result<null, string>> {
            try {
                await axios.post("/api/student/new", student);
                return NewResult.from_ok(null);
            } catch (err) {
                if(!axios.isAxiosError(err)) { return NewResult.from_err<null, string>("Non Axios Error"); }
                return NewResult.from_err<null, string>(err.response?.data);
            }
        }
        
        export async function Edit(student: ApiRequestObjects.EditStudent): Promise<Result<null, string>> {
            try {
                await axios.post("/api/student/edit", student);
                return NewResult.from_ok(null);
            } catch (err) {
                if(!axios.isAxiosError(err)) { return NewResult.from_err<null, string>("Non Axios Error"); }
                return NewResult.from_err<null, string>(err.response?.data);
            }
        }
        
        export async function Get(uuid: string, decrypt: boolean): Promise<Result<ApiResponseObjects.FullStudent, string>> {
            try {
                const response = await axios.post("/api/student/get", {uuid: uuid, decrypt: decrypt});
                return NewResult.from_ok<ApiResponseObjects.FullStudent, string>(response.data);
            } catch (err) {
                if(!axios.isAxiosError(err)) { return NewResult.from_err<ApiResponseObjects.FullStudent, string>("Non Axios Error"); }
                return NewResult.from_err<ApiResponseObjects.FullStudent, string>(err.response?.data);
            }
        }
        
        export async function Delete(uuid: string): Promise<Result<null, string>> {
            try {
                await axios.post("/api/student/delete", {uuid: uuid});
                return NewResult.from_ok<null, string>(null);
            } catch (err) {
                if(!axios.isAxiosError(err)) { return NewResult.from_err<null, string>("Non Axios Error"); }
                return NewResult.from_err<null, string>(err.response?.data);
            }
        }

        export async function Search(params: ApiRequestObjects.SearchStudent): Promise<ApiResponseObjects.FullStudent[]> {
            const response = await axios.post("/api/student/search", params);
            return response.data as ApiResponseObjects.FullStudent[];
        }

        export async function Numbers(params: number[]): Promise<Result<ApiResponseObjects.FullStudent[], string>> {
            try {
                const response = await axios.post("/api/student/numbers", {numbers: params});
                return NewResult.from_ok(response.data);
            } catch (err) {
                if(!axios.isAxiosError(err)) { return NewResult.from_err("Non Axios Error"); }
                return NewResult.from_err(err.response?.data);
            }

        }
    }

    export namespace User {
        export async function Create(user: Tables.Users): Promise<Result<{token: string}, string>> {
            try {
                const response = await axios.post("/api/auth/create", {device: "", ...user});
                return NewResult.from_ok<{token: string}, string>(response.data)
            } catch (err) {
                if(!axios.isAxiosError(err)) { return NewResult.from_err<{token: string}, string>("Non Axios Error"); }
                return NewResult.from_err<{token: string}, string>(err.response?.data)
            }
        }
        
        export async function Update(user: ApiRequestObjects.EditUser): Promise<Result<null, string>> {
            try {
                await axios.post("/api/auth/update", user);
                return NewResult.from_ok<null, string>(null)
            } catch (err) {
                if(!axios.isAxiosError(err)) { return NewResult.from_err<null, string>("Non Axios Error"); }
                return NewResult.from_err<null, string>(err.response?.data);
            }
        }
        
        export async function Signup(signup_hash: string) {
            await axios.post("/api/auth/signup", {signup_hash});
        }
        
        export async function Delete(uuid: string): Promise<Result<null, string>> {
            try {
                await axios.post("/api/auth/delete", {"uuid": uuid});
                return NewResult.from_ok<null, string>(null)
            } catch (err) {
                if(!axios.isAxiosError(err)) { return NewResult.from_err<null, string>("Non Axios Error"); }
                return NewResult.from_err<null, string>(err.response?.data)
            }
        }
        export namespace Token {
            export async function Grant(uuid: string): Promise<Result<{token: string}, string>> {
                try {
                    const response = await axios.post("/api/auth/grant", {uuid: uuid});
                    return NewResult.from_ok<{token: string}, string>(response.data)
                } catch (err) {
                    if(!axios.isAxiosError(err)) { return NewResult.from_err<{token: string}, string>("Non Axios Error"); }
                    return NewResult.from_ok<{token: string}, string>(err.response?.data);
                }
            }
            
            export async function Revoke(uuid: string): Promise<Result<null, string>> {
                try {
                    await axios.post("/api/auth/revoke", {uuid: uuid});
                    return NewResult.from_ok<null, string>(null)
                } catch (err) {
                    if(!axios.isAxiosError(err)) { return NewResult.from_err<null, string>("Non Axios Error"); }
                    return NewResult.from_err<null, string>(err.response?.data)
                }
            }
        }
    }

    export namespace Activity {
        export async function Create(activity: ApiRequestObjects.CreateActivity): Promise<Result<null, string>> {
            try {
                await axios.post("/api/activity/create", activity);
                return NewResult.from_ok<null, string>(null)
            } catch (err) {
                if(!axios.isAxiosError(err)) { return NewResult.from_err<null, string>("Non Axios Error"); }
                return NewResult.from_err<null, string>(err.response?.data)
            }
        }
    }
}



