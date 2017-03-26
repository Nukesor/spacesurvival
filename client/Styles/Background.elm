module Styles.Background exposing (..)

import Css exposing (..)
import Css.Elements exposing (..)
import Css.Namespace exposing (namespace)


type Classes
    = Background
    | Container


css =
    (stylesheet << namespace "bg")
        [ class Background
            [ width <| vw 200
            , height <| vh 200
            , backgroundImage <| url "/static/img/stardust.png"
            , backgroundRepeat repeat
            ]
        , class Container
            [ position absolute
            , top zero
            , left zero
            , overflow hidden
            , width <| vw 100
            , height <| vh 100
            , zIndex <| int -1
            ]
        ]
