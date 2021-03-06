module Main where
import AdventLib
import Data.List
import Data.List.Split

mergeLayers :: [Int] -> [Int] -> [Int]
mergeLayers = zipWith merger
    where merger 2 b = b
          merger a _ = a

main :: IO ()
main = do
    nums <- digits <$> inputFile
    let layers = chunksOf (width * height) nums
    let layerWithLeastZeroes:_ = sortOn (count 0) layers
    let num1s = count 1 layerWithLeastZeroes
    let num2s = count 2 layerWithLeastZeroes
    firstStar $ num1s * num2s

    let assembledLayers = foldr1 mergeLayers layers
    -- And now just squint at this really hard
    secondStar $ chunksOf width assembledLayers

    where width = 25
          height = 6
