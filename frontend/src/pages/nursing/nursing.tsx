import './nursing.css'
import { type JSX } from 'react'

export default function Nursing(): JSX.Element {
    return (
        <>
            <div className="title">
                <h1 onClick={() => window.location.href = "/"}>Nursing</h1>
            </div>
        </>
    )
}