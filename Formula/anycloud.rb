class Anycloud < Formula
    desc "Lambda alternative that works with multiple cloud providers"
    homepage "https://github.com/alantech/anycloud"
    url "https://github.com/alantech/anycloud/archive/v0.1.2.tar.gz"
    sha256 "3ca6c849aace024c75e5429db95b37e1a5592902557439996508b469cc706370"
    license "Apache-2.0"
  
    depends_on "rust" => :build
  
    resource "alan" do
      url "https://github.com/alantech/alan/releases/download/v0.1.30/alan-macos.tar.gz"
      sha256 "456a801d9f4c0b4e2e4e926ee2f98ea0afc12f93b4e860306cbb0bb109b5ae9d"
    end
  
    def install
      parent_dir = pwd
      ENV["PATH"] = ENV["PATH"] + ":#{pwd}"
      resource("alan").stage do
        require "fileutils"
        mv("alan", parent_dir)
      end
      Dir.chdir("./cli")
      system "cargo", "install", *std_cargo_args
    end
  
    test do
      (testpath/".anycloud/deploy.json").write <<~EOS
        { }
      EOS
      system "anycloud", "info"
    end
  end
  