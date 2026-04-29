import './Home.css'
import type { Tables } from '../../axios/structs'
import { RoleGE } from '../../components/Role';

export default function Home({user}: {user: Tables.Users}) {
    
    return (
        <>
            { RoleGE(user.role, "Admin") ?
                <div className="titleButton" onClick={() => window.location.href="/admin"}>Admin</div>
                : <></>
            }
            <div className="titleButton" onClick={() => window.location.href="/nursing"}>Nursing Numbers</div>
            <div className="titleButton" onClick={() => window.location.href="/activities"}>Activities</div>
            <div className="titleButton" onClick={() => window.location.href="/search"}>Search</div>
        </>
    )
}