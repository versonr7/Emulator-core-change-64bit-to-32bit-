#include <iostream>
#include <fstream>
#include <vector>
#include <unicorn/unicorn.h>

int main() {
    std::cout << "--- 🚀 FNAF C++ BRIDGE v0.1 ---" << std::endl;

    // 1. تهيئة المحرك
    uc_engine *uc;
    uc_err err;

    err = uc_open(UC_ARCH_ARM, UC_MODE_ARM, &uc);
    if (err) {
        std::cerr << "❌ فشل تشغيل المحرك: " << uc_strerror(err) << std::endl;
        return 1;
    }

    // 2. حجز الذاكرة (8 ميجا بايت)
    uint64_t address = 0x40000000;
    err = uc_mem_map(uc, address, 8 * 1024 * 1024, UC_PROT_ALL);
    if (err) {
        std::cerr << "❌ فشل حجز الذاكرة: " << uc_strerror(err) << std::endl;
        return 1;
    }

    // 3. قراءة ملف اللعبة
    std::ifstream file("libgdx.so", std::ios::binary | std::ios::ate);
    if (!file.is_open()) {
        std::cerr << "❌ ملف libgdx.so غير موجود!" << std::endl;
        return 1;
    }

    std::streamsize size = file.tellg();
    file.seekg(0, std::ios::beg);
    std::vector<char> buffer(size);
    if (file.read(buffer.data(), size)) {
        // كتابة الملف في الذاكرة الوهمية
        uc_mem_write(uc, address, buffer.data(), buffer.size());
        std::cout << "✅ تم تحميل libgdx.so في الذاكرة بنجاح!" << std::endl;
        std::cout << "📍 العنوان الأساسي: 0x" << std::hex << address << std::dec << std::endl;
    }

    std::cout << "--- 🕹️ الجسر جاهز للعمل ---" << std::endl;

    uc_close(uc);
    return 0;
}
