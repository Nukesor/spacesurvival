module View.Research exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Dict
import Html exposing (..)
import Html.CssHelpers
import Html.Events exposing (onClick)
import Messages exposing (Msg(StartResearching))
import Model exposing (Model)
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
        updateButton =
            if updateable model.queue model.researches research then
                [ button [ onClick (StartResearching key) ] [ Html.text "Research next level" ] ]
            else
                []
    in
        li []
            (List.concat
                [ [ Html.text ("Lv. " ++ (toString research.currentLevel) ++ " " ++ research.name) ]
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
