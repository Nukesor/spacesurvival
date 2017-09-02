module View.Resources exposing (view, rules)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Html exposing (..)
import Html.CssHelpers
import Svg exposing (image, svg)
import Svg.Attributes
import Model.Resources exposing (Resource, formatAmount)


view model =
    ul [] (List.map item model.resources)


item : Resource -> Html msg
item resource =
    li []
        [ resourceImage resource
        , Html.text ((formatAmount resource.amount) ++ "/" ++ (formatAmount resource.maxAmount) ++ " " ++ resource.name)
        ]


resourceImage resource =
    svg
        [ Svg.Attributes.width "40px"
        , Svg.Attributes.height "40px"
        ]
        [ image
            [ Svg.Attributes.xlinkHref "/static/img/minerals.svg"
            , Svg.Attributes.width "100%"
            , Svg.Attributes.height "100%"
            ]
            []
        ]


type Classes
    = Container


rules =
    (stylesheet << namespace ns) []


ns : String
ns =
    "resources"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns
