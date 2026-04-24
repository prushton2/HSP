import axios from "axios";
import type { CreateStudent, EditStudent, StudentTablesResponse } from "./structs";


export async function GetAllStudentInfo(): Promise<StudentTablesResponse> {
    const response = await axios.get('api/admin/get_student_info');
    return response.data as StudentTablesResponse
}

export async function HttpCreateStudent(student: CreateStudent) {
    const response = await axios.post("/api/admin/student/new", student);
    console.log(response);
}

export async function HttpEditStudent(student: EditStudent) {
    const response = await axios.post("/api/admin/student/edit", student);
    console.log(response);
}