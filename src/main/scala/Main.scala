package matchy

import com.google.cloud.functions.HttpFunction
import com.google.cloud.functions.HttpRequest
import com.google.cloud.functions.HttpResponse
import java.io.BufferedWriter
import java.io.IOException

class HelloWorldScala3 extends HttpFunction :
  override def service(request: HttpRequest, response: HttpResponse): Unit =
    val writer = response.getWriter
    writer.write("Hello World from Google Cloud Function in Scala 3!")
    