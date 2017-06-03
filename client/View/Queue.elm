module View.Queue exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Dict
import Html exposing (..)
import Html.CssHelpers
import Model exposing (Model)
import Model.Queue exposing (Entry)


view model =
    div [ helpers.class [ Container ] ]
        [ h3 [] [ Html.text "Queue" ]
        , ul [ helpers.class [ List ] ] (List.map (queueItem model) model.queue)
        ]


queueItem : Model -> Entry -> Html msg
queueItem model entry =
    case entry of
        Model.Queue.ResearchEntry researchEntry ->
            case Dict.get researchEntry.researchId model.researches of
                Just research ->
                    li [ helpers.class [ Item ] ] [ Html.text research.name ]

                _ ->
                    li [] []

        _ ->
            li [] []


type Classes
    = Item
    | List
    | Container


rules : Stylesheet
rules =
    (stylesheet << namespace ns)
        [ Css.class Item
            [ listStyleType none
            , borderTop3 (px 1) solid (hex "#fff")
            , padding2 (Css.em 0.5) zero
            ]
        , Css.class List
            [ paddingLeft zero
            ]
        , Css.class Container
            [ width (pct 100) ]
        ]


ns : String
ns =
    "queue"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns