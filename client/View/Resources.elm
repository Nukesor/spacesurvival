module View.Resources exposing (view, rules)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Html exposing (..)
import Html.CssHelpers
import Model.Resources exposing (Resource, formatAmount)


view model =
    ul [] (List.map item model.resources)


item : Resource -> Html msg
item resource =
    li [] [ Html.text ((formatAmount resource.amount) ++ "/" ++ (formatAmount resource.maxAmount) ++ " " ++ resource.name) ]


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
