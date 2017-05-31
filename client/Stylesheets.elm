port module Stylesheets exposing (..)

import Css.File exposing (CssCompilerProgram, CssFileStructure)
import View.Background
import View.BuildDialog
import View.Layout
import View.MenuBar


port files : CssFileStructure -> Cmd msg


fileStructure : CssFileStructure
fileStructure =
    Css.File.toFileStructure
        [ ( "elm.css"
          , Css.File.compile
                [ View.Background.rules
                , View.BuildDialog.rules
                , View.Layout.rules
                , View.MenuBar.rules
                ]
          )
        ]


main : CssCompilerProgram
main =
    Css.File.compiler files fileStructure
