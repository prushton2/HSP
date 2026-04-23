import { useState } from 'react'
import './Admin.css'

function Admin() {
    const [count, setCount] = useState(0)

    return (
    <>
        <div className="title">
        <h1>HSP Admin</h1>
        </div>

        <div className="body">
            <table>
            <tbody>
                <tr>
                    <th>Company</th>
                    <th>Contact</th>
                    <th>Country</th>
                </tr>
                <tr>
                    <td>Alfreds Futterkiste</td>
                    <td>Maria Anders</td>
                    <td>Germany</td>
                </tr>
                <tr>
                    <td>Centro comercial Moctezuma</td>
                    <td>Francisco Chang</td>
                    <td>Mexico</td>
                </tr>

            </tbody>
            </table>
        </div>
    </>
    )
}

export default Admin


// function RenderTable()