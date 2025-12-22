// Android build configuration for Tauri Mobile
plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("rust")
}

android {
    namespace = "ai.mldevops.platform"
    compileSdk = 34

    defaultConfig {
        applicationId = "ai.mldevops.platform"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "0.3.0"
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }
}

rust {
    rootDirRel = "../../../"
    targets = listOf("aarch64", "armv7", "i686", "x86_64")
}
