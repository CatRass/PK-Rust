
import SaveDisplay from "./components/SaveDisplay.js";
import SaveEditor from "./components/SaveEditor.js";
const { invoke } = window.__TAURI__.tauri

// Obtain the root 
const rootElement = document.getElementById('content')

// Create a function to wrap up your component
function App(){
    return(
        <>
            <SaveDisplay />
            <SaveEditor />
        </>
    )
}

// Use the ReactDOM.render to show your component on the browser
    ReactDOM.createRoot(rootElement).render(
        <App />,
    )