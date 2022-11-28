import i18next from "i18next";
import {initReactI18next} from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";

i18next
    .use(initReactI18next)
    .use(LanguageDetector)
    .init({
        debug: true,
        fallbackLng: "en",
        interpolation: {
            escapeValue: false
        },
        resources: {
            en: {
                translation: {
                    "title": "Sudoku Solver",
                    wrongSolutionRow: "Your solution is wrong. See row {{row}}",
                    wrongSolutionCol: "Your solution is wrong. See column {{col}}",
                    wrongSolutionBlock: "Your solution is wrong. See block {{blockRow}},{{blockCol}}",
                    incompleteSolution: "Your solution is incomplete",
                    rightSolution: "Your solution is right",
                    noSolution: "Couldn't find a solution",
                    generatedRandom: "Generated random board in {{time}}ms",
                    checkButton: "Check",
                    clearButton: "Clear",
                    generateRandomButton: "Random {{perc}}",
                    solveButton: "Solve",
                    solveStepsButton: "Solve step-by-step",
                    hideSolutionButton: "Hide solution",
                    prevStepButton: "Prev step",
                    prev10StepsButton: "Prev 10 steps",
                    nextStepButton: "Next step",
                    next10StepsButton: "Next 10 steps",
                    step: "Step {{num}}: {{message}}",
                    found: "Found solution in {{ms}}µ",
                    tried: "Tried number {{num}} in {{row}},{{col}}",
                    gaveUp: "Gave up",
                    canContainOnly: "Cell {{row}},{{col}} can only contain number {{num}}",
                    numberOnlyFitsInRow: "Number {{num}} can only be placed in one cell in row {{row}}",
                    numberOnlyFitsInCol: "Number {{num}} can only be placed in one cell in col {{col}}",
                    numberOnlyFitsInBlock: "Number {{num}} can only be placed in one cell in block {{row}},{{col}}",
                }
            },
            "pt-BR": {
                translation: {
                    title: "Solucionador de Sudoku",
                    wrongSolutionRow: "Sua solução está errada. Veja linha {{row}}",
                    wrongSolutionCol: "Sua solução está errada. Veja coluna {{col}}",
                    wrongSolutionBlock: "Sua solução está errada. Veja bloco {{blockRow}},{{blockCol}}",
                    incompleteSolution: "Sua solução está incompleta",
                    rightSolution: "Sua solução está correta",
                    noSolution: "Não foi possível encontrar uma solução",
                    generatedRandom: "Tabuleiro aleatório gerado em {{time}}ms",
                    checkButton: "Checar",
                    clearButton: "Limpar",
                    generateRandomButton: "Aleatório {{perc}}%",
                    solveButton: "Solucionar",
                    solveStepsButton: "Solucionar passo a passo",
                    hideSolutionButton: "Esconder solução",
                    prevStepButton: "Voltar passo",
                    prev10StepsButton: "Voltar 10 passos",
                    nextStepButton: "Avançar passo",
                    next10StepsButton: "Avançar 10 passos",
                    step: "Passo {{num}}: {{message}}",
                    found: "Solução encontrada em {{ms}}µ",
                    tried: "Tentar número {{num}} em {{row}},{{col}}",
                    gaveUp: "Desistir",
                    canContainOnly: "Casa {{row}},{{col}} apenas pode conter o número {{num}}",
                    numberOnlyFitsInRow: "O número {{num}} apenas pode ser colocado em uma cada na linha {{row}}",
                    numberOnlyFitsInCol: "O número {{num}} apenas pode ser colocado em uma cada na coluna {{col}}",
                    numberOnlyFitsInBlock: "O número {{num}} apenas pode ser colocado em uma cada no bloco {{row}},{{col}}",
                }
            }
        }
    });