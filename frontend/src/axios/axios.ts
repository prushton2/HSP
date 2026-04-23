import axios from "axios";
import type { StudentTablesResponse } from "./structs";


export async function GetAllStudentInfo(): Promise<StudentTablesResponse> {
    const response = await axios.get('api/admin/get_student_info');
    return response.data as StudentTablesResponse
}