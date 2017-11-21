module View.Research exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Dict
import Html exposing (..)
import Html.CssHelpers
import Html.Events exposing (onClick)
import Messages exposing (Msg(StartResearching))
import Model exposing (Model)
import Model.Queue exposing (applyQueue)
import Model.Research exposing (Research, Researches, updateable)


view : Model -> Html Messages.Msg
view model =
    div []
        [ ul []
            (model.researches
                |> Dict.map (researchItem model)
                |> Dict.values
            )
        ]


researchItem : Model -> String -> Research -> Html Messages.Msg
researchItem model key research =
    let
        ( currentLevel, upgradeText ) =
            case research.currentLevel of
                Just level ->
                    ( "Lv. " ++ (toString (level + 1)), "Research next level" )

                Nothing ->
                    ( "", "Start researching" )

        updateButton =
            if updateable (applyQueue model.researches model.queue) key then
                [ button [ onClick (StartResearching key) ] [ Html.text upgradeText ] ]
            else
                []
    in
        li []
            (List.concat
                [ [ Html.text (currentLevel ++ " " ++ research.name) ]
                , updateButton
                ]
            )


rules : Stylesheet
rules =
    (stylesheet << namespace ns)
        []


ns : String
ns =
    "research"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns
