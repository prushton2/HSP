import axios from "axios";
import type { FullStudentInfo, EditStudent, StudentTablesResponse, TableUsers, UpdateUser } from "./structs";

export namespace Http {
    export namespace Admin {
        export async function GetAllTables(): Promise<StudentTablesResponse> {
            const response = await axios.get('api/admin/all');
            return response.data as StudentTablesResponse
        }
    }
    export namespace Student {
        export async function Create(student: FullStudentInfo) {
            const response = await axios.post("/api/student/new", student);
            console.log(response);
        }
        
        export async function Edit(student: EditStudent) {
            const response = await axios.post("/api/student/edit", student);
            console.log(response);
        }
        
        export async function Get(uuid: string, decrypt: boolean): Promise<FullStudentInfo> {
            const response = await axios.post("/api/student/get", {uuid: uuid, decrypt: decrypt});
            return response.data as FullStudentInfo
        }
        
        export async function Delete(uuid: string) {
            const response = await axios.post("/api/student/delete", {uuid: uuid});
            console.log(response);
        }
    }

    export namespace User {
        export async function Create(user: TableUsers): Promise<{token: string}> {
            const response = await axios.post("/api/auth/create", {device: "", ...user});
            return response.data as {token: string}
        }
        
        export async function Update(user: UpdateUser) {
            await axios.post("/api/auth/update", user);
        }
        
        export async function Signup(signup_hash: string) {
            await axios.post("/api/auth/signup", {signup_hash});
        }
        
        export async function Delete(uuid: string) {
            await axios.post("/api/auth/delete", {"uuid": uuid});
        }
        export namespace Token {
            export async function Grant(uuid: string): Promise<{token: string}> {
                const response = await axios.post("/api/auth/grant", {uuid: uuid});
                return response.data as {token: string}
            }
            
            export async function Revoke(uuid: string) {
                const response = await axios.post("/api/auth/revoke", {uuid: uuid});
                console.log(response);
            }
        }
    }
}



