val scala3Version = "3.2.0"

lazy val root = project
  .in(file("."))
  .settings(
    name := "matchy",
    version := "0.1.0-SNAPSHOT",

    scalaVersion := scala3Version,

    libraryDependencies += "org.scalameta" %% "munit" % "0.7.29" % Test,
    // Every function needs this dependency to get the Functions Framework API.
    libraryDependencies +="com.google.cloud.functions" % "functions-framework-api" % "1.0.4",
    // To run function locally using Functions Framework's local invoker
    libraryDependencies += "com.google.cloud.functions.invoker" % "java-function-invoker" % "1.2.0" % Test,
  )
