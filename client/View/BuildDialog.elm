module View.BuildDialog exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Html.CssHelpers
import Html exposing (..)
import Html.Events exposing (..)
import Model
import Messages


view : Model.Model -> Html Messages.Msg
view model =
    case model.buildingAt of
        Just point ->
            div [ helpers.class [ Container ] ]
                [ ul
                    [ helpers.class [ BuildItemList ] ]
                    (List.map
                        (\m ->
                            li [ helpers.class [ BuildItem ] ]
                                [ Html.text m.name ]
                        )
                        model.availableModules
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
            , padding2 (Css.em 1) (zero)
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
