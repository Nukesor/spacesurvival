port module Stylesheets exposing (..)

import Css.File exposing (CssFileStructure, CssCompilerProgram)
import Styles.Background
import Styles.BuildDialog


port files : CssFileStructure -> Cmd msg


fileStructure : CssFileStructure
fileStructure =
    Css.File.toFileStructure
        [ ( "elm.css"
          , Css.File.compile
                [ Styles.Background.css
                , Styles.BuildDialog.css
                ]
          )
        ]


main : CssCompilerProgram
main =
    Css.File.compiler files fileStructure
