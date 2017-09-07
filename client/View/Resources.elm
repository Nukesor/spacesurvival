module View.Resources exposing (view, rules)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Html exposing (..)
import Html.CssHelpers
import Model.Resources exposing (Resource, formatAmount)
import Svg exposing (image, svg)
import Svg.Attributes


view model =
    ul [] (List.map item model.resources)


item : Resource -> Html msg
item resource =
    li []
        [ span [ helpers.class [ Icon ] ] [ resourceImage resource ]
        , span [] [ Html.text ((formatAmount resource.amount) ++ "/" ++ (formatAmount resource.maxAmount) ++ " " ++ resource.name) ]
        ]


resourceImage : Resource -> Html msg
resourceImage resource =
    svg
        [ Svg.Attributes.width "40px"
        , Svg.Attributes.height "40px"
        ]
        [ image
            [ Svg.Attributes.xlinkHref ("/static/img/" ++ resource.name ++ ".svg")
            , Svg.Attributes.width "100%"
            , Svg.Attributes.height "100%"
            ]
            []
        ]


type Classes
    = Container
    | Icon


rules =
    (stylesheet << namespace ns)
        [ Css.class Icon
            [ verticalAlign middle
            , display inlineBlock
            , Css.marginRight (Css.em 0.5)
            ]
        ]


ns : String
ns =
    "resources"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns
