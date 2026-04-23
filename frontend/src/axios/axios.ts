import axios from "axios";


export async function GetAllStudentInfo() {
    const response = await axios.get('api/admin/get_student_info');
    console.log(response.data);
}