import { useEffect } from "react";
import "../../App.css"
import { HttpSignupUser } from "../../axios/axios";
import { useSearchParams } from "react-router";

export default function Signup() {
    const [searchParams, _] = useSearchParams();

    useEffect(() => {
        async function init() {
            let token = searchParams.get("token")
            if (token != null) {
                try {
                    await HttpSignupUser(token)
                } catch {
                    alert("Invalid URL");
                }
            } else {
                alert("Invalid URL");
            }

            window.location.href = "/"
        }
        init();
    }, [])

    return (
    <>
        Signing in...
    </>
    )
}