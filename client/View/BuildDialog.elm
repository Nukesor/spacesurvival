module View.BuildDialog exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Dict
import Html exposing (..)
import Html.CssHelpers
import Html.Events exposing (..)
import Messages
import Model


view : Model.Model -> Html Messages.Msg
view model =
    case model.buildingAt of
        Just point ->
            div [ helpers.class [ Container ] ]
                [ ul
                    [ helpers.class [ BuildItemList ] ]
                    (model.availableModules
                        |> Dict.values
                        |> List.map
                            (\m ->
                                li [ helpers.class [ BuildItem ] ]
                                    [ Html.text m.name ]
                            )
                    )
                , cancelButton
                ]

        Nothing ->
            div [] []


cancelButton : Html Messages.Msg
cancelButton =
    button [ helpers.class [ Button ], onClick (Messages.ShowBuildDialog Nothing) ]
        [ Html.text "Cancel"
        ]


type Classes
    = Container
    | Button
    | BuildItem
    | BuildItemList


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
            , backgroundColor (rgba 0 0 0 0.4)
            ]
        , Css.class Button
            [ padding (Css.rem 0.5)
            , minWidth (Css.em 8)
            , backgroundColor (rgba 200 200 255 0.2)
            , border3 (px 1) solid (hex "#aaaacc")
            , cursor pointer
            , color inherit
            , margin (Css.rem 0.5)
            ]
        , Css.class BuildItemList
            [ paddingLeft zero
            ]
        , Css.class BuildItem
            [ listStyle none
            , padding2 (Css.em 1) (Css.em 1)
            , cursor pointer
            , hover
                [ backgroundColor (rgba 200 200 255 0.1)
                ]
            ]
        ]


ns : String
ns =
    "build-dialog"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns
