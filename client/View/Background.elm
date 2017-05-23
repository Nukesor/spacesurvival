module View.Background exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Html exposing (div)
import Html.CssHelpers exposing (withNamespace)


type Classes
    = Background
    | Container


view : Html.Html msg
view =
    div
        [ helpers.class [ Container ] ]
        [ div
            [ helpers.class [ Background ] ]
            []
        ]


rules : Stylesheet
rules =
    (stylesheet << namespace ns)
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


ns : String
ns =
    "bg"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    withNamespace ns
