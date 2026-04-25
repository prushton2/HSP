import axios from "axios";
import type { FullStudentInfo, EditStudent, StudentTablesResponse } from "./structs";


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