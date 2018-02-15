{-# LANGUAGE OverloadedStrings #-}

import Data.Monoid (mappend)
import Hakyll
import System.FilePath.Posix
  ( (</>)
  , joinPath
  , splitPath
  , takeBaseName
  , takeDirectory
  )

main :: IO ()
main =
  hakyllWith config $ do
    match "CNAME" $ do
      route idRoute
      compile copyFileCompiler
    match "images/*" $ do
      route idRoute
      compile copyFileCompiler
    match "css/*" $ do
      route idRoute
      compile compressCssCompiler
    match (fromList ["about.md"]) $ do
      route $ setExtension "html"
      compile $
        pandocCompiler >>=
        loadAndApplyTemplate "templates/default.html" defaultContext >>=
        relativizeUrls
    match "posts/*" $ do
      route $ setExtension "html"
      compile $
        pandocCompiler >>= loadAndApplyTemplate "templates/post.html" postCtx >>=
        loadAndApplyTemplate "templates/default.html" postCtx >>=
        relativizeUrls
    create ["thoughts.html"] $ do
      route idRoute
      compile $ do
        posts <- recentFirst =<< loadAll "posts/*"
        let archiveCtx =
              listField "posts" postCtx (return posts) `mappend`
              constField "title" "Thoughts" `mappend`
              defaultContext
        makeItem "" >>=
          loadAndApplyTemplate "templates/thoughts.html" archiveCtx >>=
          loadAndApplyTemplate "templates/default.html" archiveCtx >>=
          relativizeUrls
    match "index.html" $ do
      route idRoute
      compile $ do
        posts <- recentFirst =<< loadAll "posts/*"
        let indexCtx =
              listField "posts" postCtx (return posts) `mappend`
              constField "title" "Home" `mappend`
              defaultContext
        getResourceBody >>= applyAsTemplate indexCtx >>=
          loadAndApplyTemplate "templates/default.html" indexCtx >>=
          relativizeUrls
    match "templates/*" $ compile templateBodyCompiler

config :: Configuration
config =
  defaultConfiguration
  {destinationDirectory = "docs", providerDirectory = "src"}

postCtx :: Context String
postCtx = dateField "date" "%B %e, %Y" `mappend` defaultContext

cleanRoute :: Routes
cleanRoute = customRoute createIndexRoute
  where
    createIndexRoute ident =
      let p = toFilePath ident
      in takeDirectory p </> takeBaseName p </> "index.html"

srcRoute :: Routes
srcRoute = customRoute dropSrcRoute
  where
    toHTML = setExtension "html"
    dropSrcRoute ident = joinPath $ tail $ splitPath $ toFilePath ident
