import axios from "axios";
import type { FullStudentInfo, EditStudent, StudentTablesResponse, TableUsers, UpdateUser } from "./structs";


export async function GetAllStudentInfo(): Promise<StudentTablesResponse> {
    const response = await axios.get('api/admin/all');
    return response.data as StudentTablesResponse
}

export async function HttpCreateStudent(student: FullStudentInfo) {
    const response = await axios.post("/api/student/new", student);
    console.log(response);
}

export async function HttpEditStudent(student: EditStudent) {
    const response = await axios.post("/api/student/edit", student);
    console.log(response);
}

export async function HttpGetStudent(uuid: string, decrypt: boolean): Promise<FullStudentInfo> {
    const response = await axios.post("/api/student/get", {uuid: uuid, decrypt: decrypt});
    return response.data as FullStudentInfo
}

export async function HttpDeleteStudent(uuid: string) {
    const response = await axios.post("/api/student/delete", {uuid: uuid});
    console.log(response);
}

export async function HttpCreateUser(user: TableUsers): Promise<{token: string}> {
    const response = await axios.post("/api/auth/create", {device: "", ...user});
    return response.data as {token: string}
}

export async function HttpUpdateUser(user: UpdateUser) {
    await axios.post("/api/auth/update", user);
}

export async function HttpSignupUser(signup_hash: string) {
    await axios.post("/api/auth/signup", {signup_hash});
}

export async function HttpDeleteUser(uuid: string) {
    await axios.post("/api/auth/delete", {"uuid": uuid});
}

export async function HttpGrantToken(uuid: string): Promise<{token: string}> {
    const response = await axios.post("/api/auth/grant", {uuid: uuid});
    return response.data as {token: string}
}

export async function HttpRevokeTokens(uuid: string) {
    const response = await axios.post("/api/auth/revoke", {uuid: uuid});
    console.log(response);
}