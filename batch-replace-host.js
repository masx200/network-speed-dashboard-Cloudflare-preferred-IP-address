import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

// 获取当前文件的目录路径
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const oldTextArray = [
  "4444.cloudflare.182682.xyz",
  "4444.cloudflare.182682.xyz",
  "4444.cloudflare.182682.xyz",

];

for (const oldText of oldTextArray) {
  // 配置要替换的文本

  const newText = "4444.cloudflare.182682.xyz";

  // 支持的文件扩展名
  const extensions = [".js", ".yml", ".json", ".txt", ".md"];

  // 递归遍历目录
  function walkDirectory(dir, callback) {
    const files = fs.readdirSync(dir);

    for (const file of files) {
      const filePath = path.join(dir, file);
      const stat = fs.statSync(filePath);

      if (stat.isDirectory()) {
        // 跳过 node_modules 目录
        if (file !== "node_modules" && file !== ".git") {
          walkDirectory(filePath, callback);
        }
      } else if (stat.isFile()) {
        callback(filePath);
      }
    }
  }

  // 替换文件内容
  function replaceInFile(filePath) {
    const ext = path.extname(filePath);

    if (!extensions.includes(ext)) {
      return false;
    }

    try {
      let content = fs.readFileSync(filePath, "utf8");

      if (content.includes(oldText)) {
        console.log(`正在处理: ${filePath}`);

        // 替换文本
        const newContent = content.replace(
          new RegExp(escapeRegExp(oldText), "g"),
          newText,
        );

        // 写入文件
        fs.writeFileSync(filePath, newContent, "utf8");

        console.log(`✅ 已更新: ${filePath}`);
        return true;
      }
    } catch (error) {
      console.error(`❌ 处理文件时出错 ${filePath}:`, error.message);
    }

    return false;
  }

  // 转义正则表达式特殊字符
  function escapeRegExp(string) {
    return string.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  }

  // 主函数
  function main() {
    console.log("🔄 开始批量替换...");
    console.log(`📝 替换内容: "${oldText}" → "${newText}"`);
    console.log(`📂 搜索目录: ${__dirname}`);
    console.log("");

    let processedCount = 0;
    let updatedCount = 0;

    // 遍历当前目录
    walkDirectory(__dirname, (filePath) => {
      processedCount++;
      if (replaceInFile(filePath)) {
        updatedCount++;
      }
    });

    console.log("");
    console.log("✨ 批量替换完成!");
    console.log(`📊 处理文件总数: ${processedCount}`);
    console.log(`🔧 更新文件数量: ${updatedCount}`);

    if (updatedCount === 0) {
      console.log("ℹ️  没有找到需要替换的内容");
    }
  }

  // 运行程序
  main();
}
