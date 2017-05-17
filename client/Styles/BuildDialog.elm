module Styles.BuildDialog exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Html.CssHelpers


type Classes
    = Container


ns : String
ns =
    "build-dialog"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns


css : Stylesheet
css =
    (stylesheet << namespace ns)
        [ class Container
            [ border3 (px 1) solid (hex "#FFF")
            , position absolute
            , transform (translate2 (pct -50) (pct -50))
            , top (pct 50)
            , left (pct 50)
            , padding (px 10)
            ]
        ]
