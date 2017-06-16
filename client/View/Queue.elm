module View.Queue exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Dict
import Html exposing (..)
import Html.CssHelpers
import Model exposing (Model)
import Model.Queue exposing (Entry, timeToCompletion)


view : Model -> Html msg
view model =
    div [ helpers.class [ Container ] ]
        [ h3 [] [ Html.text "Queue" ]
        , ul [ helpers.class [ List ] ] (List.map (queueItem model) model.queue)
        ]


queueItem : Model -> Entry -> Html msg
queueItem model entry =
    let
        ( name, level ) =
            getInfo model entry

        remainingTimeString =
            timeToCompletion entry model.currentDate
                |> Maybe.map toString
                |> Maybe.map (\time -> time ++ " secs Remaining")
                |> Maybe.withDefault ""
    in
        li [ helpers.class [ Item ] ]
            [ Html.text <| "Lv. " ++ (toString level) ++ " " ++ name
            , br [] []
            , Html.text remainingTimeString
            ]


getInfo : Model -> Entry -> ( String, Int )
getInfo model entry =
    case entry of
        Model.Queue.ResearchEntry entry ->
            let
                name =
                    case Dict.get entry.researchId model.researches of
                        Just research ->
                            research.name

                        Nothing ->
                            ""
            in
                ( name, entry.level )

        Model.Queue.ModuleEntry entry ->
            let
                name =
                    case Dict.get entry.moduleId model.availableModules of
                        Just mod ->
                            mod.name

                        Nothing ->
                            ""
            in
                ( name, entry.level )


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
