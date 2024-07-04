
export default function SaveEditor() {
    return(
        <div id="saveEditor">
            <div id="pokemon" data-is-party="none" data-is-box="none" data-index="-1">
                <img id="displayPkmn"/>
                <input type="text" id="name" maxlength="11"/>
                <p id="level"></p>
                <div id="moves">
                    <div id="move-1" class="moveBox"></div>
                    <div id="move-2" class="moveBox"></div>
                    <div id="move-3" class="moveBox"></div>
                    <div id="move-4" class="moveBox"></div>
                </div>

                {/* <!-- <object data="./assets/stats-display/base.svg" type="image/svg+xml" id="statWheel"></object> --> */}
                <svg
                    version="1.1"
                    width="300" height="300"
                    viewBox="0 0 350 350"
                    xmlns="http://www.w3.org/2000/svg"
                    id="statWheel"
                >

                    <polygon 
                        id="base" 
                        points = ""
                        stroke="transparent" 
                        fill="#D2D2D0" 
                        stroke-width="5"

                        data-hp=""
                        data-atk=""
                        data-def=""
                        data-spd=""
                        data-spc=""
                    />
                    
                    <polyline 
                        id="stats" 
                        points = ""
                        stroke="transparent" 
                        fill="#9699E8" 
                        stroke-width="5"
                    />

                    {/* <script href="./scripts/stats-display-script.js" type="module">
                    </script> */}
                </svg>

            </div>
        </div>
    )
}