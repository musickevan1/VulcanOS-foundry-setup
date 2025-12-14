/* Wofi Style - ${THEME_NAME} Theme */

* {
    font-family: "JetBrainsMono Nerd Font", monospace;
    font-size: 14px;
}

window {
    margin: 0px;
    border: 2px solid ${ACCENT};
    border-radius: 10px;
    background-color: ${BG_PRIMARY};
}

#input {
    margin: 10px;
    border: none;
    border-radius: 8px;
    color: ${FG_PRIMARY};
    background-color: ${BG_SECONDARY};
    padding: 10px;
}

#input:focus {
    border: 2px solid ${ACCENT};
}

#inner-box {
    margin: 5px;
    border: none;
    background-color: transparent;
}

#outer-box {
    margin: 5px;
    border: none;
    background-color: transparent;
}

#scroll {
    margin: 0px;
    border: none;
}

#text {
    margin: 5px;
    border: none;
    color: ${FG_PRIMARY};
}

#entry {
    margin: 2px;
    padding: 8px;
    border-radius: 6px;
    background-color: transparent;
}

#entry:selected {
    background-color: ${SELECTION};
    border: none;
}

#entry:hover {
    background-color: ${BG_TERTIARY};
}

#img {
    margin-right: 10px;
}
