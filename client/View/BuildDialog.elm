module View.BuildDialog exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Html.CssHelpers
import Html exposing (..)
import Model


type Classes
    = Container


view : Model.Model -> Html msg
view model =
    case model.buildingAt of
        Just point ->
            div [ css.class [ Container ] ]
                [ ul
                    []
                    (List.map (\m -> li [] [ Html.text m.name ]) model.availableModules)
                ]

        Nothing ->
            div [] []


ns : String
ns =
    "build-dialog"


css : Html.CssHelpers.Namespace String class id msg
css =
    Html.CssHelpers.withNamespace ns


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns


rules : Stylesheet
rules =
    (stylesheet << namespace ns)
        [ Css.class Container
            [ border3 (px 1) solid (hex "#FFF")
            , position absolute
            , transform (translate2 (pct -50) (pct -50))
            , top (pct 50)
            , left (pct 50)
            , padding (px 10)
            ]
        ]
