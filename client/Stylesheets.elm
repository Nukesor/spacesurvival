port module Stylesheets exposing (..)

import Css.File exposing (CssFileStructure, CssCompilerProgram)
import View.Background
import View.BuildDialog


port files : CssFileStructure -> Cmd msg


fileStructure : CssFileStructure
fileStructure =
    Css.File.toFileStructure
        [ ( "elm.css"
          , Css.File.compile
                [ View.Background.rules
                , View.BuildDialog.rules
                ]
          )
        ]


main : CssCompilerProgram
main =
    Css.File.compiler files fileStructure
