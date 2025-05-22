import java.io.{File, FileInputStream, FileOutputStream}
import java.nio.file.{Files, Paths, StandardCopyOption}
import scala.sys.process._

object DownloadCache {
  val downloadsDir = System.getProperty("user.home") + "/Downloads"
  val cacheDir = System.getProperty("user.home") + "/DownloadCache"
  // Path to the malicious detector executable (relative or absolute)
  val detectorPath = "filesafe/malicious_detector" // or "filesafe/malicious_detector.exe" on Windows

  def main(args: Array[String]): Unit = {
    val downloads = new File(downloadsDir).listFiles
    if (downloads == null) {
      println(s"No files found in $downloadsDir or directory does not exist.")
      return
    }
    new File(cacheDir).mkdirs()

    downloads.filter(_.isFile).foreach { file =>
      val destFile = new File(cacheDir, file.getName)
      if (!destFile.exists()) {
        println(s"Caching new file: ${file.getName}")

        // Call the C malicious file detector
        val detectorResult = try {
          Seq(detectorPath, file.getAbsolutePath).!!
        } catch {
          case e: Throwable =>
            println(s"Error running malicious detector: ${e.getMessage}")
            ""
        }

        if (detectorResult.trim == "OK") {
          Files.copy(file.toPath, destFile.toPath, StandardCopyOption.REPLACE_EXISTING)
          println(s"File ${file.getName} cached.")
        } else if (detectorResult.trim == "MALICIOUS") {
          println(s"Malicious file detected: ${file.getName} - Not cached!")
        } else {
          println(s"Could not determine if file is malicious: ${file.getName} - Not cached!")
        }
      }
    }
  }
}
